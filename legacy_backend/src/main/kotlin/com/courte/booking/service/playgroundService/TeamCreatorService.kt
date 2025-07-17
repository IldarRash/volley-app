package com.courte.booking.service.playgroundService

import com.courte.booking.persistence.entity.Player
import com.courte.booking.persistence.entity.PlayerPosition
import com.courte.booking.persistence.entity.Position
import com.courte.booking.persistence.entity.Type


interface TeamCreatorService {
    fun createTeams(players: List<Player>, teamNumber: Int): Map<String, List<Player>>
}

class AmountScoreService : TeamCreatorService {
    override fun createTeams(players: List<Player>, teamNumber: Int): Map<String, List<Player>> {
        val setAndOthers = findSetters(players, teamNumber)

        val mapTeams = mutableMapOf<String, List<Player>>()
        var count = 1
        setAndOthers.first.forEach {
            mapTeams.put("Team$count", listOf(it))
            count++
        }
        var currentPlayers = setAndOthers.second
        while (countSteps(mapTeams, players.size)) {
            val pair = findMaxAndDeletePlayer(currentPlayers)
            val minimalTeam = mapTeams.map { it.key to it.value.map { player -> player.score }.sum() }.minBy { it.second }
            mapTeams.compute(minimalTeam.first,{ _, v -> v!!.plus(pair.first) })

            currentPlayers = pair.second
        }

        return mapTeams
    }

    fun findMaxAndDeletePlayer(players: List<Player>) : Pair<Player, List<Player>> {
        val maxPlayer = players.maxBy { it.score }
        return Pair(maxPlayer, players.filter { !it.equals(maxPlayer) })
    }
    fun countSteps(mapTeams: Map<String, List<Player>>, playersSize: Int) =
        mapTeams.map { it.value.size }.sum() != playersSize

    fun countAmountOfTeam(players: List<Player>) : Int  =
        players.map { it.score }.sum()

    fun findSetters(players: List<Player>, teamNumber: Int): Pair<List<Player>, List<Player>> {
        val setters = players.filter { it.positions.contains(PlayerPosition(Type.Side, Position.Setter)) }.stream().limit(teamNumber.toLong()).toList()
        val other = players.minus(setters)
        return Pair(setters, other)
    }
}

class IntelegenceService : TeamCreatorService {
    override fun createTeams(players: List<Player>, teamNumber: Int): Map<String, List<Player>> {
        TODO("Not yet implemented")
    }

}