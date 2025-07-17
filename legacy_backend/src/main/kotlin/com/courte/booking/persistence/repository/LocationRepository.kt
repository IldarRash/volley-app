package com.courte.booking.persistence.repository

import com.courte.booking.persistence.entity.Location
import org.springframework.data.mongodb.repository.ReactiveMongoRepository
import reactor.core.publisher.Mono
import java.util.*

interface LocationRepository : ReactiveMongoRepository<Location, UUID> {

}