package com.realworld.spring.webflux.dto.request

import javax.validation.constraints.NotBlank

data class CreateCommentRequest(
    @field:NotBlank
    val body: String,
) {
    fun toComment(commentId: String, authorId: Long) = Comment(
        id = commentId,
        authorId = authorId,
        body = body
    )
}
