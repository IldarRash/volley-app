package com.realworld.spring.webflux.persistence.entity

import com.realworld.spring.webflux.dto.view.LocationView
import com.realworld.spring.webflux.dto.view.LocationViewFull
import com.realworld.spring.webflux.dto.view.LocationViewShort
import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import java.util.UUID

@Document
data class Location(
    @Id val id: UUID,
    val name: String,
    val address: String,
    val latitude: Float,
    val longitude: Float,
    val landLordName: String,
    val phoneNumber: String
)

fun Location.toLocationView() = LocationViewShort(name, address, latitude, longitude)

fun Location.toLocationViewFull() = LocationViewFull(id, name, address, latitude, longitude, landLordName, phoneNumber)