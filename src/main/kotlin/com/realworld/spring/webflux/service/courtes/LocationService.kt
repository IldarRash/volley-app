package com.realworld.spring.webflux.service.courtes

import com.realworld.spring.webflux.dto.request.LocationRequestCreate
import com.realworld.spring.webflux.dto.request.LocationRequestUpdate
import com.realworld.spring.webflux.dto.request.toEntity
import com.realworld.spring.webflux.persistence.entity.toLocationViewFull
import com.realworld.spring.webflux.persistence.repository.LocationRepository
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