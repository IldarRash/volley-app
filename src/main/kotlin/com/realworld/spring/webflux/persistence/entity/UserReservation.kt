package com.realworld.spring.webflux.persistence.entity

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document

@Document
data class UserReservation (
        @Id val id: Long,
        val userId: Long,
        val bookedId: Long
        )