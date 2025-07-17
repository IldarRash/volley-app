package com.courte.booking.dto

import com.courte.booking.dto.view.UserViewPlayer
import com.courte.booking.dto.view.UserViewShort
import com.courte.booking.persistence.entity.*
import java.util.UUID

data class User(
    val id: UUID? = null,
    val username: String,
    val encodedPassword: String,
    val email: String,
    val score: Int = 0,
    val positions: List<PlayerPosition> = listOf(),
    val playerProps: List<PlayerProps> = listOf(),
    val gender: Gender,
    val bio: String? = null,
    val image: String? = null,
) {
    fun toUserEntity() = UserEntity(
        id = this.id ?: UUID.randomUUID(),
        username = this.username,
        encodedPassword = this.encodedPassword,
        email = this.email,
        bio = this.bio,
        image = this.image,
        gender = gender,
        positions = positions,
        playerProps = playerProps,
        score = score
    )

    fun toUserViewShort(token: String) = UserViewShort(
        email = this.email,
        token = token,
        username = this.username,
        bio = this.bio,
        image = this.image,
        positions = this.positions
    )

    fun toUserViewPlayerFull() = UserViewPlayer(
            email = this.email,
            username = this.username,
            bio = this.bio,
            image = this.image,
            positions = this.positions,
            score = this.score,
            props = playerProps
    )

    fun toPlayer() = Player(
        username,
        score,
        positions,
        playerProps,
        false
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