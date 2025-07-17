package com.courte.booking.service.user

interface UserTokenProvider {
    fun getToken(userId: String): String
}