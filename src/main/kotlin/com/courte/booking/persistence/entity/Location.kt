package com.courte.booking.persistence.entity

import com.courte.booking.dto.view.LocationViewFull
import com.courte.booking.dto.view.LocationViewShort
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