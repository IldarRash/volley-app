package com.courte.booking.api

import com.courte.booking.dto.request.BookCourtCreateRequest
import com.courte.booking.dto.request.BookCourtUpdateRequest
import com.courte.booking.dto.request.LocationRequestCreate
import com.courte.booking.dto.request.LocationRequestUpdate
import com.courte.booking.dto.view.BookCourtView
import com.courte.booking.dto.view.LocationView
import com.courte.booking.service.courtes.BookCourtsService
import com.courte.booking.service.courtes.LocationService
import com.realworld.spring.webflux.api.*
import com.realworld.spring.webflux.user.UserSessionProvider
import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.*
import javax.validation.Valid


@RestController
@RequestMapping("/api")
class CourtController(val locationService: LocationService, val bookCourtsService: BookCourtsService, val userSessionProvider: UserSessionProvider) {


    @PostMapping("/location")
    @ResponseStatus(HttpStatus.CREATED)
    suspend fun createLocation(@RequestBody @Valid request: LocationWrapper<LocationRequestCreate>): LocationWrapper<LocationView> {
        return locationService.addLocation(request.content).toLocationWrapper()
    }

    @PutMapping("/location")
    suspend fun updateLocation(@RequestBody @Valid request: LocationWrapper<LocationRequestUpdate>): LocationWrapper<LocationView> {
        return locationService.updateLocation(request.content).toLocationWrapper()
    }

    @GetMapping("/location")
    suspend fun locations(): LocationsWrapper<LocationView> {
        return locationService.getAllLocation().toLocationsWrapper()
    }

    @PostMapping("/book/court")
    @ResponseStatus(HttpStatus.CREATED)
    suspend fun createBooking(@RequestBody @Valid request: BookCourtWrapper<BookCourtCreateRequest>): BookCourtWrapper<BookCourtView> =
        bookCourtsService.addBooking(request.content).toBookCourtWrapper()

    @PutMapping("/book/court")
    suspend fun updateBooking(@RequestBody @Valid request: BookCourtWrapper<BookCourtUpdateRequest>): BookCourtWrapper<BookCourtView> =
        bookCourtsService.updateBooking(request.content).toBookCourtWrapper()


   @GetMapping("/book/court")
   suspend fun getAllBooking(): BookCourtsWrapper<BookCourtView> =
       bookCourtsService.getAllLocation().toBookCourtsWrapper()

}