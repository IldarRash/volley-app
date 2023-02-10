package com.courte.booking.api

import com.courte.booking.dto.request.LocationRequestCreate
import com.courte.booking.dto.request.LocationRequestUpdate
import com.courte.booking.dto.view.LocationView
import com.courte.booking.service.courtes.BookCourtsService
import com.courte.booking.service.courtes.LocationService
import com.realworld.spring.webflux.api.toLocationWrapper
import com.realworld.spring.webflux.api.toLocationsWrapper
import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.*
import javax.validation.Valid


@RestController
@RequestMapping("/api")
class CourtController(val locationService: LocationService, val bookCourtsService: BookCourtsService) {


    @PostMapping("/location")
    @ResponseStatus(HttpStatus.CREATED)
    suspend fun createLocation(@RequestBody @Valid request: com.realworld.spring.webflux.api.LocationWrapper<LocationRequestCreate>): com.realworld.spring.webflux.api.LocationWrapper<LocationView> {
        return locationService.addLocation(request.content).toLocationWrapper()
    }

    @PutMapping("/location")
    suspend fun updateLocation(@RequestBody @Valid request: com.realworld.spring.webflux.api.LocationWrapper<LocationRequestUpdate>): com.realworld.spring.webflux.api.LocationWrapper<LocationView> {
        return locationService.updateLocation(request.content).toLocationWrapper()
    }

    @GetMapping("/location")
    suspend fun locations(): com.realworld.spring.webflux.api.LocationsWrapper<LocationView> {
        return locationService.getAllLocation().toLocationsWrapper()
    }
}