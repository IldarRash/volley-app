package com.courte.booking.persistence.repository

import com.courte.booking.dto.User
import com.courte.booking.exceptions.InvalidRequestException
import com.courte.booking.persistence.entity.UserEntity
import kotlinx.coroutines.flow.map
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.reactive.asFlow
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.reactor.awaitSingleOrNull
import org.springframework.stereotype.Component
import org.springframework.transaction.annotation.Transactional
import reactor.core.publisher.Flux
import reactor.core.publisher.Mono
import java.util.*

@Component
class UserDataService(
    private val userRepository: UserRepository,
) {
    @Transactional
    suspend fun save(user: User): User {
        val userEntity = userRepository.save(user.toUserEntity()).awaitSingle()
        return userEntity.toUser()
    }

    suspend fun findById(userId: UUID): User {
        val userEntity = userRepository.findById(userId).awaitSingle()
        return toUser(userEntity = userEntity)
    }

    suspend fun findByEmail(email: String): User {
        val userEntity = userRepository.findByEmail(email).awaitSingle()
        return toUser(userEntity = userEntity)
    }

    suspend fun existsByEmail(email: String) = userRepository.existsByEmail(email)

    suspend fun existsByUsername(username: String) = userRepository.existsByUsername(username)

    suspend fun findByUsername(username: String): User? {
        val userEntity = userRepository.findByUsername(username).awaitSingleOrNull()
        return if (userEntity != null) toUser(userEntity = userEntity) else null
    }

    suspend fun findByUsernameOrFail(username: String): User {
        val userEntity = userRepository.findByUsername(username)
            .switchIfEmpty(Mono.error(InvalidRequestException("Username", "not found")))
            .awaitSingle()
        return toUser(userEntity = userEntity)
    }

    suspend fun findByEmailOrFail(email: String): User {
        val userEntity = userRepository.findByEmail(email)
            .switchIfEmpty(Mono.error(InvalidRequestException("Email", "not found")))
            .awaitSingle()
        return toUser(userEntity = userEntity)
    }

    suspend fun saveRoleToUser(user: User) {
        userRepository.save(user.toUserEntity())
    }

    suspend fun getAllUsers(): Flux<User> = userRepository.findAll().map { toUser(userEntity = it) }

    private fun toUser(userEntity: UserEntity): User {
        return userEntity.toUser()
    }
}