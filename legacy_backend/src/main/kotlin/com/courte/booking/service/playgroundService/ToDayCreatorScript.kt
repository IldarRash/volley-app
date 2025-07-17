package com.courte.booking.service.playgroundService

import com.realworld.spring.webflux.service.FileReaderService

fun main() {
    val readerService = FileReaderService("players.txt")
    val teamCreatorService = AmountScoreService()


    val players = readerService.getPlayers()

    val teams = teamCreatorService.createTeams(players.filter { it.isToday }, 3)

    val builder = StringBuilder()
    val strings = teams.map { it.key to it.value.joinToString { "${it.username} 8===D -- ${it.score}\n" } }
    strings.forEach {
        builder.append("${it.first} \n ${it.second} \n\n\n")
    }

    println(builder.toString())
}