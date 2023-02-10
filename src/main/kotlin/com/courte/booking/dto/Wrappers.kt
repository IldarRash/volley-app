package com.realworld.spring.webflux.api

import com.courte.booking.dto.view.LocationView
import com.courte.booking.dto.view.UserView
import com.fasterxml.jackson.annotation.JsonProperty

data class UserWrapper<T>(@JsonProperty("user") val content: T)

data class LocationWrapper<T>(@JsonProperty("location") val content: T)
data class LocationsWrapper<T>(@JsonProperty("locations") val content: List<T>)

data class BookCourtWrapper<T>(@JsonProperty("bookCourt") val content: T)
data class AddPlayerWrapper<T>(@JsonProperty("addPlayer") val content: T)

data class ProfilesWrapper<T>(@JsonProperty("profiles") val content: List<T>)

fun <T> T.toUserWrapper() = UserWrapper(this)
fun <T> T.toLocationWrapper() = LocationWrapper(this)
fun <T> T.toBookCourtWrapper() = BookCourtWrapper(this)
fun <T> T.toAddPlayerWrapper() = AddPlayerWrapper(this)

fun List<LocationView>.toLocationsWrapper() = LocationsWrapper(this)
fun List<UserView>.toProfileMapper() = ProfilesWrapper(this)