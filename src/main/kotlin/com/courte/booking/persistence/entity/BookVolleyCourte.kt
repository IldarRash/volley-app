package com.courte.booking.persistence.entity

import com.courte.booking.dto.view.BookCourtFullView
import com.courte.booking.dto.view.BookCourtViewShort
import com.courte.booking.dto.view.LocationView
import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import java.time.Instant
import java.util.*

data class Team(val number: Int, val players: List<Player>, val score: Int)

@Document
data class BookCourt(
        @Id val id: UUID,
        val locationId: UUID,
        val startAt: Instant = Instant.now(),
        val endAt: Instant = Instant.now(),
        val allPlayers: List<UUID> = listOf(),
        val teams: List<Team> = listOf(),
        val limit: Int,
        val startBooking: Instant = Instant.now(),
        val description: String,
        val price: Int
)

fun BookCourt.toShortView(location: LocationView) =
        BookCourtViewShort(
                id,
                startAt,
                limit,
                description,
                price,
                location,
                allPlayers.size
        )


fun BookCourt.toFullView(location: LocationView, players: List<Player>) =
        BookCourtFullView(
                id,
                startAt,
                limit,
                description,
                price,
                location,
                players,
                teams,
                allPlayers.size
        )