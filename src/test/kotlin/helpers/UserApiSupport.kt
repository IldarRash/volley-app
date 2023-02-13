package helpers

import com.courte.booking.dto.request.UpdateUserRequest
import com.courte.booking.dto.request.UserAuthenticationRequest
import com.courte.booking.dto.request.UserRegistrationRequest
import com.courte.booking.dto.view.UserView
import com.realworld.spring.webflux.api.UserWrapper
import com.realworld.spring.webflux.api.toUserWrapper

import org.springframework.test.web.reactive.server.WebTestClient
import org.springframework.test.web.reactive.server.expectBody

class UserApiSupport(private val client: WebTestClient) {

    fun signup(request: UserRegistrationRequest = UserSamples.sampleUserRegistrationRequest()): UserView = client.post()
        .uri("/api/users")
        .bodyValue(request.toUserWrapper())
        .exchange()
        .expectBody<UserWrapper<UserView>>()
        .returnResult()
        .responseBody!!
        .content

    fun login(request: UserAuthenticationRequest = UserSamples.sampleUserAuthenticationRequest()): UserView =
        client.post()
            .uri("/api/users/login")
            .bodyValue(request.toUserWrapper())
            .exchange()
            .expectBody<UserWrapper<UserView>>()
            .returnResult()
            .responseBody!!
            .content

    fun currentUser(token: String): UserView = client.get()
        .uri("/api/user")
        .authorizationToken(token)
        .exchange()
        .expectBody<UserWrapper<UserView>>()
        .returnResult()
        .responseBody!!
        .content

    fun updateUser(token: String, request: UpdateUserRequest): UserView = client.put()
        .uri("/api/user")
        .bodyValue(request.toUserWrapper())
        .authorizationToken(token)
        .exchange()
        .expectBody<UserWrapper<UserView>>()
        .returnResult()
        .responseBody!!
        .content
}