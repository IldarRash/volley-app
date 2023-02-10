package com.realworld.spring.webflux.persistence.repository

import com.realworld.spring.webflux.persistence.entity.BookCourt
import org.springframework.data.mongodb.repository.ReactiveMongoRepository

interface BookCourtRepository : ReactiveMongoRepository<BookCourt, Long> {
}