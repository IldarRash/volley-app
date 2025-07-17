package com.courte.booking.api

import com.courte.booking.dto.request.AdminUserRequest
import com.courte.booking.dto.request.UpdateUserRequest
import com.courte.booking.dto.request.UserAuthenticationRequest
import com.courte.booking.dto.request.UserRegistrationRequest
import com.courte.booking.dto.view.UserView
import com.courte.booking.service.user.UserService
import com.realworld.spring.webflux.api.ProfilesWrapper
import com.realworld.spring.webflux.api.UserWrapper
import com.realworld.spring.webflux.api.toProfileMapper
import com.realworld.spring.webflux.api.toUserWrapper
import com.realworld.spring.webflux.user.UserSessionProvider
import kotlinx.coroutines.flow.toList
import kotlinx.coroutines.reactive.asFlow
import org.springframework.http.HttpStatus
import org.springframework.security.access.prepost.PreAuthorize
import org.springframework.web.bind.annotation.*
import javax.validation.Valid

@RestController
@RequestMapping("/api")
class UserController(private val userService: UserService, private val userSessionProvider: UserSessionProvider) {

    @PostMapping("/users")
    @ResponseStatus(HttpStatus.CREATED)
    suspend fun signup(@RequestBody @Valid request: UserWrapper<UserRegistrationRequest>): UserWrapper<UserView> {
        return userService.signup(request.content).toUserWrapper()
    }

    @PostMapping("/users/login")
    @ResponseStatus(HttpStatus.CREATED)
    suspend fun login(@RequestBody @Valid request: UserWrapper<UserAuthenticationRequest>): UserWrapper<UserView> {
        return userService.login(request.content).toUserWrapper()
    }

    @GetMapping("/user")
    suspend fun getCurrentUser(): UserWrapper<UserView> {
        val (user, token) = userSessionProvider.getCurrentUserSessionOrFail()
        return user.toUserViewShort(token).toUserWrapper()
    }

    @PutMapping("/user")
    suspend fun updateUser(@RequestBody @Valid request: UserWrapper<UpdateUserRequest>): UserWrapper<UserView> {
        val userContext = userSessionProvider.getCurrentUserSessionOrFail()
        return userService.updateUser(request.content, userContext).toUserWrapper()
    }

    @PutMapping("/admin/user")
    @PreAuthorize("hasAuthority('Admin')")
    suspend fun updateUserAdmin(@RequestBody @Valid request: UserWrapper<AdminUserRequest>): UserWrapper<UserView> {
        val userContext = userSessionProvider.getCurrentUserSessionOrFail()
        return userService.updateUserByAdmin(request.content, userContext).toUserWrapper()
    }

    @GetMapping("/profiles/admin/all")
    @PreAuthorize("hasAuthority('Admin')")
    suspend fun listProfiles(): ProfilesWrapper<UserView> {
        return userService.getAllUsers().asFlow().toList().toProfileMapper()
    }
    @GetMapping("/user/addRole")
    suspend fun addRole(): UserWrapper<UserView> {
        val currentUser = userSessionProvider.getCurrentUserOrNull()!!
        return userService.addRole(currentUser).run { currentUser }.toUserViewPlayerFull().toUserWrapper()
    }
}