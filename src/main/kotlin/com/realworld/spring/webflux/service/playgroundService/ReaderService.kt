package com.realworld.spring.webflux.service

import com.realworld.spring.webflux.dto.User
import com.realworld.spring.webflux.persistence.entity.*
import com.realworld.spring.webflux.service.user.UserService
import java.io.File

interface ReaderService {
    fun getPlayers(): List<Player>
}
class FileReaderService(val filePass: String): ReaderService {
    override fun getPlayers(): List<Player> =
        getFileStrings(filePass)
            .map { parseString(it) }

    fun getFileStrings(pass: String): List<String> =
        File("src/main/resources/$pass").useLines { it.toList() }


    fun parseString(playStr: String): Player {
        val props = playStr.split(",")


        return Player(
            props[0],
            props[2].toInt(),
            takePosition(props[1]),
            takeProps(props[3], props[4], props[5], props[6], props[7]),
            props.size > 8
        )
    }

    fun takePosition(positions: String): List<PlayerPosition> {
        return positions.split(" ")
            .map {
                when(it) {
                    "DI" -> PlayerPosition(Type.Side, Position.OutsideHitter)
                    "OP" -> PlayerPosition(Type.Side, Position.OppositeHitter)
                    "SE" -> PlayerPosition(Type.Side, Position.Setter)
                    "L" -> PlayerPosition(Type.Side, Position.Libero)
                    "NO" -> PlayerPosition(Type.Side, Position.NO)
                    "CO" -> PlayerPosition(Type.Side, Position.MiddleBlocker)
                    else -> PlayerPosition(Type.Side, Position.NO)
                }
            }
    }

    fun takeProps(attac: String,
                  defend: String,
                  serve: String,
                  set: String,
                  block: String
    ): List<PlayerProps> =
        mapOf(
            attac to Property.Attack,
            block to Property.Block,
            serve to Property.Serve,
            set to Property.Set,
            defend to Property.Receive
        )
            .map { PlayerProps(it.key.toInt(), it.value) }
}


