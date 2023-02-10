package com.realworld.spring.webflux.dto.view

import com.realworld.spring.webflux.persistence.entity.Player
import com.realworld.spring.webflux.persistence.entity.Team
import java.time.Instant
import java.util.*

interface BookCourtView {

    fun startTime(): Date

    fun location(): LocationView

    fun limit(): Int

    fun description(): String

    fun price(): Int
}

data class BookCourtViewShort(
    val startAt: Instant,
    val limit: Int,
    val description: String,
    val price: Int,
    val location: LocationView,
    val currentCount: Int
) : BookCourtView {
    override fun startTime(): Date = Date.from(startAt)

    override fun location(): LocationView = location

    override fun limit(): Int = limit

    override fun description(): String = description

    override fun price(): Int = price

}

data class BookCourtFullView(
    val startAt: Instant,
    val limit: Int,
    val description: String,
    val price: Int,
    val location: LocationView,
    val players: List<Player>,
    val teams: List<Team>,
    val currentCount: Int
) : BookCourtView {
    override fun startTime(): Date = Date.from(startAt)

    override fun location(): LocationView = location

    override fun limit(): Int = limit

    override fun description(): String = description

    override fun price(): Int = price
}