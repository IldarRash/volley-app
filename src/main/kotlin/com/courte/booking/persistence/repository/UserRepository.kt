package com.courte.booking.persistence.repository

import com.courte.booking.persistence.entity.UserEntity
import org.springframework.data.mongodb.repository.ReactiveMongoRepository
import org.springframework.data.repository.reactive.ReactiveCrudRepository
import reactor.core.publisher.Mono
import java.util.UUID


interface UserRepository : ReactiveMongoRepository<UserEntity, UUID> {
    fun findByEmail(email: String): Mono<UserEntity>

    fun existsByEmail(email: String): Mono<Boolean>

    fun existsByUsername(username: String): Mono<Boolean>

    fun findByUsername(username: String): Mono<UserEntity>
}

