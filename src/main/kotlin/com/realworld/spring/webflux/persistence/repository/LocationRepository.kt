package com.realworld.spring.webflux.persistence.repository

import com.realworld.spring.webflux.persistence.entity.Location
import com.realworld.spring.webflux.persistence.entity.UserEntity
import org.springframework.data.mongodb.repository.ReactiveMongoRepository
import reactor.core.publisher.Mono
import java.util.*

interface LocationRepository : ReactiveMongoRepository<Location, UUID> {

}