package com.realworld.spring.webflux.dto.view

import com.realworld.spring.webflux.persistence.entity.PlayerPosition
import com.realworld.spring.webflux.persistence.entity.PlayerProps

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


