package com.courte.booking.persistence.entity

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import java.time.Instant

data class Team(val number: Int, val players: List<Player>, val score: Int)

@Document
data class BookCourt(
        @Id val id: Long,
        val locationId: Long,
        val startAt: Instant = Instant.now(),
        val endAt: Instant = Instant.now(),
        val allPlayers: List<Long> = listOf(),
        val teams: List<Team> = listOf(),
        val limit: Int,
        val startBooking: Instant = Instant.now(),
        val description: String,
        val price: Int
)
