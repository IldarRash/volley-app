package com.realworld.spring.webflux.persistence.entity

import org.springframework.data.annotation.Id
import org.springframework.data.mongodb.core.mapping.Document

@Document
data class Location(
        @Id val id: Long,
        val name: String,
        val address: String,
        val latitude: Float,
        val longitude: Float
)