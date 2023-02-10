package com.realworld.spring.webflux.user


import com.courte.booking.dto.User
import com.courte.booking.exceptions.InvalidRequestException
import com.courte.booking.persistence.repository.UserDataService
import com.courte.booking.security.TokenPrincipal
import kotlinx.coroutines.reactor.awaitSingle
import kotlinx.coroutines.reactor.awaitSingleOrNull
import org.springframework.security.core.context.ReactiveSecurityContextHolder
import org.springframework.stereotype.Component
import java.util.*

@Component
class UserSessionProvider(private val userDataService: UserDataService) {

    suspend fun getCurrentUserOrNull(): User? = getCurrentUserSessionOrNull()?.user

    suspend fun getCurrentUserOrFail(): User = getCurrentUserSessionOrFail().user

    suspend fun getCurrentUserSessionOrFail() =
        getCurrentUserSessionOrNull() ?: throw InvalidRequestException("User", "current user is not login in")

    suspend fun getCurrentUserSessionOrNull(): UserSession? {
        val context = ReactiveSecurityContextHolder.getContext().awaitSingleOrNull() ?: return null
        val tokenPrincipal = context.authentication.principal as TokenPrincipal
        val user = userDataService.findById(UUID.fromString(tokenPrincipal.userId))
        return UserSession(user, tokenPrincipal.token)
    }
}

data class UserSession(
    val user: User,
    val token: String,
)