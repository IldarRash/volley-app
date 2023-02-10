package com.realworld.spring.webflux.dto.request

import com.realworld.spring.webflux.persistence.entity.Team
import java.time.Instant

data class BookCourtCreateRequest(
    val locationId: Long,
    val startAt: Instant,
    val endAt: Instant,
    val limit: Int,
    val startBooking: Instant = Instant.now(),
    val description: String,
    val price: Int
)


data class BookCourtUpdateRequest(
    val id: Long,
    val locationId: Long,
    val startAt: Instant,
    val endAt: Instant,
    val allPlayers: List<Long>,
    val teams: List<Team>,
    val limit: Int,
    val startBooking: Instant = Instant.now(),
    val description: String,
    val price: Int
)

data class AddPlayer(
    val CourtId: Long
)