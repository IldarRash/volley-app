package com.courte.booking.service.user


import com.courte.booking.dto.User
import com.courte.booking.dto.request.AdminUserRequest
import com.courte.booking.dto.request.UpdateUserRequest
import com.courte.booking.dto.request.UserAuthenticationRequest
import com.courte.booking.dto.request.UserRegistrationRequest
import com.courte.booking.dto.view.UserView
import com.courte.booking.exceptions.InvalidRequestException
import com.courte.booking.persistence.repository.UserDataService
import kotlinx.coroutines.reactor.awaitSingle
import org.springframework.stereotype.Component

@Component
class SecuredUserService(
    private val userDataService: UserDataService,
    private val passwordService: PasswordService,
    private val userTokenProvider: UserTokenProvider,
) {

    suspend fun signup(request: UserRegistrationRequest): UserView {
        if (userDataService.existsByEmail(request.email).awaitSingle()) {
            throw emailAlreadyInUseException()
        }
        if (userDataService.existsByUsername(request.username).awaitSingle()) {
            throw usernameAlreadyInUseException()
        }
        val encodedPassword = passwordService.encodePassword(request.password)
        val user = request.toUser(encodedPassword)
        val savedUser = userDataService.save(user)
        return createAuthenticationResponse(savedUser)
    }

    suspend fun login(request: UserAuthenticationRequest): UserView {
        val user = userDataService.findByEmailOrFail(request.email)
        if (!passwordService.matchesRawPasswordWithEncodedPassword(
                rawPassword = request.password, encodedPassword = user.encodedPassword)) {
            throw InvalidRequestException("Password", "invalid")
        }
        return createAuthenticationResponse(user)
    }

    suspend fun prepareUserForUpdate(request: UpdateUserRequest, user: User): User {
        return User(
            id = user.id,
            bio = request.bio ?: user.bio,
            image = request.image?: user.image,
            encodedPassword = request.password
                ?. let {passwordService.encodePassword(request.password) }
                ?: user.encodedPassword,
            username = resolveUsername(user, request.username) ,
            email = resolveEmail(user, request.email),
            gender = user.gender,
            positions = user.positions
        )
    }

    suspend fun prepareUserForAdmin(request: AdminUserRequest, user: User): User {
        return User(
                id = user.id,
                bio = request.bio ?: user.bio,
                image = request.image?: user.image,
                encodedPassword = user.encodedPassword,
                username = resolveUsername(user, request.username) ,
                email = resolveEmail(user, request.email),
                score = request.score,
                positions = request.positions,
                playerProps = request.playerProps,
                gender = request.gender
        )
    }

    private suspend fun resolveUsername(user: User, newUsername: String?): String {
        if (newUsername == null || newUsername.isEmpty()) {
            return user.username
        }
        if (user.username == newUsername) {
            return newUsername
        }
        if (userDataService.existsByUsername(newUsername).awaitSingle()) {
            throw usernameAlreadyInUseException()
        }
        return newUsername
    }

    private suspend fun resolveEmail(user: User, newEmail: String?): String {
        if (newEmail == null || newEmail.isEmpty()) {
            return user.email
        }
        if (user.email == newEmail) {
            return newEmail
        }
        if (userDataService.existsByEmail(newEmail).awaitSingle()) {
            throw emailAlreadyInUseException()
        }
        return newEmail
    }

    private fun createAuthenticationResponse(user: User): UserView {
        val token = userTokenProvider.getToken(user.id.toString())
        return user.toUserViewShort(token)
    }

    private fun usernameAlreadyInUseException() = InvalidRequestException("Username", "already in use")

    private fun emailAlreadyInUseException() = InvalidRequestException("Email", "already in use")
}