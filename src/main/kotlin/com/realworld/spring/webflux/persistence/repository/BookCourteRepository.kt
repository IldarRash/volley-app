package com.realworld.spring.webflux.persistence.repository

import com.realworld.spring.webflux.persistence.entity.BookVolleyCourt
import org.springframework.data.mongodb.repository.ReactiveMongoRepository

interface BookCourtRepository : ReactiveMongoRepository<BookVolleyCourt, Long> {
}