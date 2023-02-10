package com.courte.booking.persistence.entity

import com.courte.booking.dto.User
import org.apache.commons.lang3.StringUtils
import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document
import org.springframework.data.relational.core.mapping.Column
import org.springframework.data.relational.core.mapping.Table
import reactor.core.publisher.Mono
import java.util.*

enum class Role {
    User, Admin
}

enum class Gender {
    Male, Female, Other
}

@Document("app_user")
data class UserEntity(

        @Id val id: UUID?,
        val username: String,
        val encodedPassword: String,
        val email: String,
        val bio: String? = null,
        val image: String? = null,
        val roles: List<Role> = listOf(Role.User),
        val score: Int,
        val positions: List<PlayerPosition> = listOf(),
        val playerProps: List<PlayerProps> = listOf(),
        val gender: Gender
) {
    fun toUser() = User(
       id = this.id,
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
}
