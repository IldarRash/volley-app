package com.realworld.spring.webflux.dto.request

import com.realworld.spring.webflux.persistence.entity.Location
import java.util.*

data class LocationRequestCreate(
    val name: String,
    val address: String,
    val latitude: Float,
    val longitude: Float,
    val landLordName: String,
    val phoneNumber: String
)

fun LocationRequestCreate.toEntity() =
    Location(UUID.randomUUID(), name, address, latitude, longitude, landLordName, phoneNumber )


data class LocationRequestUpdate(
    val id: UUID,
    val name: String,
    val address: String,
    val latitude: Float,
    val longitude: Float,
    val landLordName: String,
    val phoneNumber: String
)

fun LocationRequestUpdate.toEntity() =
    Location(id, name, address, latitude, longitude, landLordName, phoneNumber )
