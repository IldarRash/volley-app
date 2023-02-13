package helpers

import com.courte.booking.dto.User
import com.courte.booking.dto.request.UpdateUserRequest
import com.courte.booking.dto.request.UserAuthenticationRequest
import com.courte.booking.dto.request.UserRegistrationRequest
import com.courte.booking.persistence.entity.Gender
import com.courte.booking.service.user.PasswordService
import java.util.*


object UserSamples {
    const val SAMPLE_USERNAME = "Test username"
    const val SAMPLE_EMAIL = "testemail@gmail.com"
    const val SAMPLE_PASSWORD = "testpassword"
    val SAMPLE_USER_ID = UUID.randomUUID().mostSignificantBits and Long.MAX_VALUE
    private val passwordService = PasswordService()

    fun sampleUserRegistrationRequest() = UserRegistrationRequest(
        username = SAMPLE_USERNAME,
        email = SAMPLE_EMAIL,
        password = SAMPLE_PASSWORD,
        gender = Gender.Male
    )

    fun sampleUserAuthenticationRequest() = UserAuthenticationRequest(
        email = SAMPLE_EMAIL,
        password = SAMPLE_PASSWORD,
    )

    fun sampleUser(passwordService: PasswordService) = User(
        id = UUID.randomUUID(),
        username = SAMPLE_USERNAME,
        email = SAMPLE_EMAIL,
        encodedPassword = passwordService.encodePassword(SAMPLE_PASSWORD),
        image = "test image url",
        bio = "test bio",
        gender = Gender.Male
    )

    fun sampleUser() = sampleUser(passwordService)

    fun sampleUpdateUserRequest() = UpdateUserRequest(
        bio = "new bio",
        email = "newemail@gmail.com",
        image = "new image",
        username = "new username",
        password = "new password",
    )
}