package helpers


import com.courte.booking.security.JwtConfig
import com.courte.booking.security.JwtSigner
import com.courte.booking.security.SecurityConfig
import com.courte.booking.security.TokenFormatter
import org.springframework.context.annotation.Import

@Import(SecurityConfig::class, TokenFormatter::class, JwtSigner::class, JwtConfig::class)
@Target(AnnotationTarget.CLASS)
@Retention(AnnotationRetention.RUNTIME)
annotation class ImportAppSecurity
