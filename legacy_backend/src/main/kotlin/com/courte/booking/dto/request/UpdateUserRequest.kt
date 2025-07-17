package com.courte.booking.dto.request

import com.courte.booking.persistence.entity.Gender
import com.courte.booking.persistence.entity.PlayerPosition
import com.courte.booking.persistence.entity.PlayerProps
import com.courte.booking.validation.NotBlankOrNull
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
