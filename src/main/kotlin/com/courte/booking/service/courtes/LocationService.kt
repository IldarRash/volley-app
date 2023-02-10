package com.courte.booking.service.courtes

import com.courte.booking.dto.request.LocationRequestCreate
import com.courte.booking.dto.request.LocationRequestUpdate
import com.courte.booking.dto.request.toEntity
import com.courte.booking.persistence.entity.toLocationViewFull
import com.courte.booking.persistence.repository.LocationRepository
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.reactive.asFlow
import kotlinx.coroutines.reactor.awaitSingle
import org.springframework.stereotype.Component


@Component
class LocationService(val locationRepository: LocationRepository) {

    suspend fun addLocation(locationRequest: LocationRequestCreate) =
        locationRepository.save(locationRequest.toEntity())
            .map { it.toLocationViewFull() }.awaitSingle()


    suspend fun updateLocation(locationRequest: LocationRequestUpdate) =
        locationRepository.save(locationRequest.toEntity())
            .map { it.toLocationViewFull() }.awaitSingle()

    suspend fun getAllLocation() =
        locationRepository.findAll()
            .map { it.toLocationViewFull() }
            .asFlow()
            .toList()
}