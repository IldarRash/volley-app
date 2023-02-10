package com.realworld.spring.webflux.api

import com.realworld.spring.webflux.dto.request.LocationRequestCreate
import com.realworld.spring.webflux.dto.request.LocationRequestUpdate
import com.realworld.spring.webflux.dto.request.UpdateUserRequest
import com.realworld.spring.webflux.dto.request.UserAuthenticationRequest
import com.realworld.spring.webflux.dto.view.LocationView
import com.realworld.spring.webflux.dto.view.UserView
import com.realworld.spring.webflux.service.courtes.BookCourtsService
import com.realworld.spring.webflux.service.courtes.LocationService
import org.springframework.http.HttpStatus
import org.springframework.web.bind.annotation.*
import javax.validation.Valid


@RestController
@RequestMapping("/api")
class CourtController(val locationService: LocationService, val bookCourtsService: BookCourtsService) {


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
}