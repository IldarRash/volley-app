package com.realworld.spring.webflux.persistence.entity

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import org.springframework.data.mongodb.core.mapping.Field



enum class Position {
    OutsideHitter, OppositeHitter, MiddleBlocker, Setter, Libero
}
enum class Type {
    Main, Side
}

data class PlayerPosition(
        val type: Type,
        val position: Position
)
enum class Property {
    Attack, Block, Serve, Set, Receive
}
data class PlayerProps(
        val score: Int = 0,
        val prop: Property

)

data class Player (
        val positions: List<PlayerPosition> = listOf(),
        val playerProps: List<PlayerProps> = listOf()
)