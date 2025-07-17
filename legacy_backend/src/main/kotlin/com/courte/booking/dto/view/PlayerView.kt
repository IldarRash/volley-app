package com.courte.booking.dto.view

import com.courte.booking.persistence.entity.PlayerPosition
import com.courte.booking.persistence.entity.PlayerProps

interface PlayerView {
    fun positions(): List<PlayerPosition>
}

data class ShortPlayerView(val positions: List<PlayerPosition>) : PlayerView {
    override fun positions(): List<PlayerPosition> = positions
}


data class FullPlayerView(
        val positions: List<PlayerPosition>,
        val playerProps: List<PlayerProps>
): PlayerView {
    override fun positions(): List<PlayerPosition> = positions
}


