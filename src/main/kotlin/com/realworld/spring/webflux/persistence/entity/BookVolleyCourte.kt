package com.realworld.spring.webflux.persistence.entity

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import java.time.Instant
import java.util.Date

data class Team(val number: Int, val players: List<Player>, val score: Int)

@Document
data class BookVolleyCourt(
        @Id val id: Long,
        val locationId: Long,
        val startAt: Instant = Instant.now(),
        val endAt: Instant = Instant.now(),
        val allPlayers: List<Long>,
        val teams: List<Team>,
        val limit: Int,
        val startBooking: Instant = Instant.now(),
        val description: String,
        val price: Int
)
