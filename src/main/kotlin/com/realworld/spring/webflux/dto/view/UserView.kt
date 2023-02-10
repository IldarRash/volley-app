package com.realworld.spring.webflux.dto.view

import com.realworld.spring.webflux.dto.User
import com.realworld.spring.webflux.persistence.entity.*


interface UserView {
    fun name(): String
    fun email(): String
}
data class UserViewShort(
    val email: String,
    val token: String,
    val username: String,
    val bio: String?,
    val image: String?,
    val positions: List<PlayerPosition>
) : UserView {
    override fun name(): String  = username
    override fun email(): String = email
}


data class UserViewPlayer(
        val email: String,
        val username: String,
        val bio: String?,
        val image: String?,
        val positions: List<PlayerPosition>,
        val score: Int,
        val props: List<PlayerProps>
) : UserView {
    override fun name(): String  = username
    override fun email(): String = email
}

