package com.courte.booking.api

import com.realworld.spring.webflux.persistence.repository.UserDataService
import com.realworld.spring.webflux.persistence.repository.UserRepository
import helpers.UserApiSupport
import helpers.UserSamples
import kotlinx.coroutines.runBlocking
import kotlinx.coroutines.test.runTest
import org.assertj.core.api.Assertions.assertThat
import org.junit.jupiter.api.BeforeEach
import org.junit.jupiter.api.Test
import org.springframework.beans.factory.annotation.Autowired
import org.springframework.boot.test.context.SpringBootTest
import org.springframework.test.web.reactive.server.WebTestClient

@SpringBootTest(webEnvironment = SpringBootTest.WebEnvironment.RANDOM_PORT)
class UserApiTest(
    @Autowired val client: WebTestClient,
    @Autowired val userDataService: UserDataService,
    @Autowired val userRepository: UserRepository
) {

    val userApi = UserApiSupport(client)

    @BeforeEach
    fun setUp() {
        userRepository.deleteAll().block()
    }
}