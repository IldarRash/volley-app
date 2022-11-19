package com.realworld.spring.webflux.service.user

import com.realworld.spring.webflux.dto.request.UpdateUserRequest
import com.realworld.spring.webflux.dto.request.UserAuthenticationRequest
import com.realworld.spring.webflux.dto.request.UserRegistrationRequest
import com.realworld.spring.webflux.dto.view.*
import com.realworld.spring.webflux.dto.User
import com.realworld.spring.webflux.persistence.repository.UserDataService
import com.realworld.spring.webflux.user.UserSession
import org.springframework.stereotype.Component

@Component
class UserService(
    private val userDataService: UserDataService,
    private val securedUserService: SecuredUserService,
) {

    suspend fun signup(request: UserRegistrationRequest): UserView {
        return securedUserService.signup(request)
    }

    suspend fun login(request: UserAuthenticationRequest): UserView {
        return securedUserService.login(request)
    }

    suspend fun updateUser(request: UpdateUserRequest, userSession: UserSession): UserView {
        val (user, token) = userSession
        val userToSave = securedUserService.prepareUserForUpdate(request, user)
        val savedUser = userDataService.save(userToSave)
        return savedUser.toUserViewShort(token)
    }

    suspend fun getProfile(username: String, viewer: User?): UserView =
        userDataService.findByUsernameOrFail(username).toUserViewShort("")

}