package com.courte.booking.dto.view

import com.courte.booking.persistence.entity.PlayerPosition
import com.courte.booking.persistence.entity.PlayerProps


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

