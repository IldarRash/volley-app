package com.realworld.spring.webflux.dto.request

import com.realworld.spring.webflux.persistence.entity.Gender
import com.realworld.spring.webflux.persistence.entity.Player
import com.realworld.spring.webflux.persistence.entity.PlayerPosition
import com.realworld.spring.webflux.persistence.entity.PlayerProps
import com.realworld.spring.webflux.validation.NotBlankOrNull
import javax.validation.constraints.Email

data class UpdateUserRequest(
    @field:Email
    @field:NotBlankOrNull
    val email: String?,
    @field:NotBlankOrNull
    val username: String?,
    @field:NotBlankOrNull
    val password: String?,
    val positions: List<PlayerPosition> = listOf(),
    val image: String?,
    val bio: String?,
)


data class AdminUserRequest(
    @field:Email
    @field:NotBlankOrNull
    val email: String?,
    @field:NotBlankOrNull
    val username: String?,
    val image: String?,
    val bio: String?,
    val score: Int,
    val positions: List<PlayerPosition> = listOf(),
    val playerProps: List<PlayerProps> = listOf(),
    val gender: Gender
)
