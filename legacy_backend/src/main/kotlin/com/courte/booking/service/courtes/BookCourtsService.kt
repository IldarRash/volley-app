package com.courte.booking.service.courtes

import com.courte.booking.dto.request.*
import com.courte.booking.dto.view.BookCourtFullView
import com.courte.booking.dto.view.BookCourtView
import com.courte.booking.persistence.entity.*
import com.courte.booking.persistence.repository.BookCourtRepository
import com.courte.booking.persistence.repository.LocationRepository
import com.courte.booking.service.user.UserService
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.reactive.asFlow
import kotlinx.coroutines.reactive.awaitSingle
import kotlinx.coroutines.reactor.awaitSingle
import org.springframework.stereotype.Component
import org.springframework.stereotype.Repository
import reactor.core.publisher.Mono
import reactor.util.function.Tuple3
import java.util.UUID

@Component
class BookCourtsService(val bookingRepository: BookCourtRepository,
                        val locationRepository: LocationRepository,
                        val userService: UserService) {

    suspend fun addBooking(bookCourtCreateRequest: BookCourtCreateRequest ) =
        bookingRepository.save(bookCourtCreateRequest.toEntity())
            .flatMap { getWithLocation(it) }
            .map { it.second.toShortView(it.first.toLocationView()) }.awaitSingle()


    suspend fun updateBooking(bookiCourtUpdateRequest: BookCourtUpdateRequest): BookCourtFullView {
        val court = bookingRepository.save(bookiCourtUpdateRequest.toEntity()).awaitSingle()

        return getWithLocationAndTeams(court)
    }

    suspend fun getAllLocation(): List<BookCourtView> =
        bookingRepository.findAll()
            .flatMap { getWithLocation(it) }
            .map {  it.second.toShortView(it.first.toLocationView()) }
            .asFlow()
            .toList()

    suspend fun addPlayerToBooking(player: UUID, bookId: UUID) =
        bookingRepository.findById(bookId)
            .flatMap { bookingRepository.save(it.copy(allPlayers = it.allPlayers.plus(player))) }
            .flatMap { getWithLocation(it) }
            .map{ it.second.toShortView(it.first.toLocationView())}
            .awaitSingle()


   fun getWithLocation(court: BookCourt) =
       locationRepository.findById(court.locationId)
           .map { Pair(it, court) }

    suspend fun getWithLocationAndTeams(court: BookCourt): BookCourtFullView {
        val location = locationRepository.findById(court.id).awaitSingle()
        val players = userService.getAllPayersByIds(court.allPlayers).map { it.toPlayer() }.asFlow().toList()

        return court.toFullView(location.toLocationViewFull(), players)
    }
}