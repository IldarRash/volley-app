package com.realworld.spring.webflux.dto

import com.realworld.spring.webflux.dto.view.UserViewPlayer
import com.realworld.spring.webflux.dto.view.UserViewShort
import com.realworld.spring.webflux.persistence.entity.Gender
import com.realworld.spring.webflux.persistence.entity.Player
import com.realworld.spring.webflux.persistence.entity.UserEntity

class User(
    val id: Long? = null,
    val username: String,
    val encodedPassword: String,
    val email: String,
    val player: Player,
    val gender: Gender,
    val bio: String? = null,
    val image: String? = null,
) {
    fun toUserEntity() = UserEntity(
        id = this.id,
        username = this.username,
        encodedPassword = this.encodedPassword,
        email = this.email,
        bio = this.bio,
        image = this.image,
        gender = gender,
        player = player
    )

    fun toUserViewShort(token: String) = UserViewShort(
        email = this.email,
        token = token,
        username = this.username,
        bio = this.bio,
        image = this.image,
    )

    fun toUserViewPlayer() = UserViewPlayer(
            email = this.email,
            username = this.username,
            bio = this.bio,
            image = this.image,
            player = this.player
    )

    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as User

        if (id != other.id) return false

        return true
    }

    override fun hashCode(): Int {
        return id.hashCode()
    }

    override fun toString(): String {
        return "User(id='$id', username='$username', encodedPassword='$encodedPassword', email='$email', bio='$bio', image='$image'"
    }


}