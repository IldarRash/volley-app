package com.realworld.spring.webflux.api

import com.fasterxml.jackson.annotation.JsonProperty
import com.realworld.spring.webflux.dto.view.UserView

data class UserWrapper<T>(@JsonProperty("user") val content: T)

data class ProfilesWrapper<T>(@JsonProperty("profiles") val content: List<T>)

fun <T> T.toUserWrapper() = UserWrapper(this)
fun List<UserView>.toProfileMapper() = ProfilesWrapper(this)