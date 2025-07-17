package com.courte.booking.dto.request

import com.courte.booking.persistence.entity.BookCourt
import com.courte.booking.persistence.entity.Team
import java.time.Instant
import java.util.UUID

data class BookCourtCreateRequest(
    val locationId: UUID,
    val startAt: Instant,
    val endAt: Instant,
    val limit: Int,
    val startBooking: Instant = Instant.now(),
    val description: String,
    val price: Int
)

fun BookCourtCreateRequest.toEntity() =
    BookCourt(
        id = UUID.randomUUID(),
        locationId = locationId,
        startAt = startAt,
        endAt = endAt,
        limit = limit,
        startBooking = startBooking,
        description = description,
        price = price
    )



data class BookCourtUpdateRequest(
    val id: UUID,
    val locationId: UUID,
    val startAt: Instant,
    val endAt: Instant,
    val allPlayers: List<UUID>,
    val teams: List<Team>,
    val limit: Int,
    val startBooking: Instant = Instant.now(),
    val description: String,
    val price: Int
)

fun BookCourtUpdateRequest.toEntity() =
    BookCourt(
        id = UUID.randomUUID(),
        locationId = locationId,
        startAt = startAt,
        endAt = endAt,
        limit = limit,
        startBooking = startBooking,
        description = description,
        price = price,
        allPlayers = allPlayers,
        teams = teams
    )

data class AddPlayer(
    val CourtId: UUID
)