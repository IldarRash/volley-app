package com.courte.booking.dto.view

import java.util.*

interface LocationView {
    fun name(): String
    fun address(): String
    fun latitude(): Float
    fun longitude(): Float
}
data class LocationViewShort(
    val name: String,
    val address: String,
    val latitude: Float,
    val longitude: Float
): LocationView {
    override fun name(): String = name

    override fun address(): String = address

    override fun latitude(): Float = latitude

    override fun longitude(): Float = longitude
}

data class LocationViewFull(
    val id: UUID,
    val name: String,
    val address: String,
    val latitude: Float,
    val longitude: Float,
    val landLordName: String,
    val phoneNumber: String
) : LocationView {
    override fun name(): String = name

    override fun address(): String = address

    override fun latitude(): Float = latitude

    override fun longitude(): Float = longitude
}