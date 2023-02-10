package com.courte.booking.persistence.repository

import com.courte.booking.persistence.entity.BookCourt
import org.springframework.data.mongodb.repository.ReactiveMongoRepository

interface BookCourtRepository : ReactiveMongoRepository<BookCourt, Long> {
}