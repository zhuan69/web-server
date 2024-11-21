use super::header::{self, HeaderKeys, Headers};

pub(crate) struct ResponseHttp<'a> {
    pub content: String,
    pub code: HttpResponseStatus,
    pub headers: &'a mut Headers,
}

pub(crate) enum HttpResponseStatus {
    StatusOK(u16, String),
    StatusCreated(u16, String),
    StatusAccepted(u16, String),
    StatusNonAuthoritativeInfomation(u16, String),
    StatusNoContent(u16, String),
    StatusResetContent(u16, String),
    StatusPartialContent(u16, String),
    StatusMultipleChoices(u16, String),
    StatusMovedPermanently(u16, String),
    StatusFound(u16, String),
    StatusSeeOther(u16, String),
    StatusNoModified(u16, String),
    StatusUseProxy(u16, String),
    StatusUnused(u16, String),
    StatusTemporaryRedirect(u16, String),
    StatusBadRequest(u16, String),
    StatusUnauthorized(u16, String),
    StatusPaymentRequired(u16, String),
    StatusForbidden(u16, String),
    StatusNotFound(u16, String),
    StatusMethodNotAllowed(u16, String),
    StatusNotAcceptable(u16, String),
    StatusProxyAuthenticationRequired(u16, String),
    StatusRequestTimeout(u16, String),
    StatusConflict(u16, String),
    StatusGone(u16, String),
    StatusLengthRequired(u16, String),
    StatusPreconditionFailed(u16, String),
    StatusRequestEntityTooLarge(u16, String),
    StatusRequestUriTooLong(u16, String),
    StatusUnsupportedMediaType(u16, String),
    StatusRequestedRangeNotSatisfiable(u16, String),
    StatusExpectationFailed(u16, String),
    StatusInternalServerError(u16, String),
    StatusNotImplemented(u16, String),
    StatusBadGateway(u16, String),
    StatusServiceUnavailable(u16, String),
    StatusGatewayTimeout(u16, String),
    StatusHttpVersionNotSupported(u16, String),
}

impl HttpResponseStatus {
    pub(crate) fn status_http_version_not_supported() -> Self {
        HttpResponseStatus::StatusHttpVersionNotSupported(505, "Http Not Supported".to_string())
    }

    pub(crate) fn status_gateway_timeout() -> Self {
        HttpResponseStatus::StatusGatewayTimeout(504, "Gateway Timeout".to_string())
    }

    pub(crate) fn status_service_unavailable() -> Self {
        HttpResponseStatus::StatusServiceUnavailable(503, "Service Unavailable".to_string())
    }

    pub(crate) fn status_bad_gateway() -> Self {
        HttpResponseStatus::StatusBadGateway(502, "Bad Gateway".to_string())
    }

    pub(crate) fn status_not_implemented() -> Self {
        HttpResponseStatus::StatusNotImplemented(501, "Not Implemented".to_string())
    }

    pub(crate) fn status_internal_server_error() -> Self {
        HttpResponseStatus::StatusInternalServerError(500, "Internal Server Error".to_string())
    }

    pub(crate) fn status_expectation_failed() -> Self {
        HttpResponseStatus::StatusExpectationFailed(417, "Expectation Failed".to_string())
    }

    pub(crate) fn status_requested_range_not_satisfiable() -> Self {
        HttpResponseStatus::StatusRequestedRangeNotSatisfiable(
            416,
            "Requested Range Not Satisfied".to_string(),
        )
    }

    pub(crate) fn status_unsupported_media_type() -> Self {
        HttpResponseStatus::StatusUnsupportedMediaType(415, "Unsupported Media Type".to_string())
    }

    pub(crate) fn status_request_uri_too_long() -> Self {
        HttpResponseStatus::StatusRequestUriTooLong(414, "Request Uri Too Long".to_string())
    }

    pub(crate) fn status_request_entity_too_large() -> Self {
        HttpResponseStatus::StatusRequestEntityTooLarge(413, "Request Entity Too Large".to_string())
    }

    pub(crate) fn status_precondition_failed() -> Self {
        HttpResponseStatus::StatusPreconditionFailed(412, "Precondition Failed".to_string())
    }

    pub(crate) fn status_length_required() -> Self {
        HttpResponseStatus::StatusLengthRequired(411, "Length Required".to_string())
    }

    pub(crate) fn status_gone() -> Self {
        HttpResponseStatus::StatusGone(410, "Gone".to_string())
    }

    pub(crate) fn status_conflict() -> Self {
        HttpResponseStatus::StatusConflict(409, "Conflict".to_string())
    }

    pub(crate) fn status_request_timeout() -> Self {
        HttpResponseStatus::StatusRequestTimeout(408, "Request Timeout".to_string())
    }

    pub(crate) fn status_proxy_authentication_required() -> Self {
        HttpResponseStatus::StatusProxyAuthenticationRequired(
            407,
            "Proxy Authentication".to_string(),
        )
    }

    pub(crate) fn status_not_acceptable() -> Self {
        HttpResponseStatus::StatusNotAcceptable(406, "Not Acceptable".to_string())
    }

    pub(crate) fn status_method_not_allowed() -> Self {
        HttpResponseStatus::StatusMethodNotAllowed(405, "Method Not Allowed".to_string())
    }

    pub(crate) fn status_not_found() -> Self {
        HttpResponseStatus::StatusNotFound(404, "Not Found".to_string())
    }

    pub(crate) fn status_forbidden() -> Self {
        HttpResponseStatus::StatusForbidden(403, "Forbidden".to_string())
    }

    pub(crate) fn status_payment_required() -> Self {
        HttpResponseStatus::StatusPaymentRequired(402, "Payment Required".to_string())
    }

    pub(crate) fn status_unauthorized() -> Self {
        HttpResponseStatus::StatusUnauthorized(401, "Unauthorized".to_string())
    }

    pub(crate) fn status_bad_request() -> Self {
        HttpResponseStatus::StatusBadRequest(400, "Bad Request".to_string())
    }

    pub(crate) fn status_temporary_redirect() -> Self {
        HttpResponseStatus::StatusTemporaryRedirect(307, "Temporary Redirect".to_string())
    }

    pub(crate) fn status_unused() -> Self {
        HttpResponseStatus::StatusUnused(306, "Unused".to_string())
    }

    pub(crate) fn status_use_proxy() -> Self {
        HttpResponseStatus::StatusUseProxy(305, "Use Proxy".to_string())
    }

    pub(crate) fn status_no_modified() -> Self {
        HttpResponseStatus::StatusNoModified(304, "No Modified".to_string())
    }

    pub(crate) fn status_see_other() -> Self {
        HttpResponseStatus::StatusSeeOther(303, "See Other".to_string())
    }

    pub(crate) fn status_found() -> Self {
        HttpResponseStatus::StatusFound(302, "Found".to_string())
    }

    pub(crate) fn status_moved_permanently() -> Self {
        HttpResponseStatus::StatusMovedPermanently(301, "Moved Permanently".to_string())
    }

    pub(crate) fn status_multiple_choices() -> Self {
        HttpResponseStatus::StatusMultipleChoices(300, "Multiple Choices".to_string())
    }

    pub(crate) fn status_partial_content() -> Self {
        HttpResponseStatus::StatusPartialContent(206, "Partial Content".to_string())
    }

    pub(crate) fn status_reset_content() -> Self {
        HttpResponseStatus::StatusResetContent(205, "Reset Content".to_string())
    }

    pub(crate) fn status_no_content() -> Self {
        HttpResponseStatus::StatusNoContent(204, "No Content".to_string())
    }

    pub(crate) fn status_non_authoritative_infomation() -> Self {
        HttpResponseStatus::StatusNonAuthoritativeInfomation(
            203,
            "Not Authoritative Information".to_string(),
        )
    }

    pub(crate) fn status_ok() -> Self {
        HttpResponseStatus::StatusOK(200, "OK".to_string())
    }

    pub(crate) fn status_accepted() -> Self {
        HttpResponseStatus::StatusAccepted(202, "Accepted".to_string())
    }
}

impl<'a> ResponseHttp<'a> {
    pub(super) fn new_response_http(
        headers: &'a mut Headers,
        content: Option<String>,
        status: Option<HttpResponseStatus>,
    ) -> Self {
        ResponseHttp {
            content: content.unwrap_or(String::new()),
            code: status.unwrap_or(HttpResponseStatus::status_ok()),
            headers,
        }
    }

    pub(super) fn response_format(&mut self) -> String {
        let (code, message) = match &self.code {
            HttpResponseStatus::StatusOK(value, message) => (value, message),
            HttpResponseStatus::StatusCreated(value, message) => (value, message),
            HttpResponseStatus::StatusAccepted(value, message) => (value, message),
            HttpResponseStatus::StatusNonAuthoritativeInfomation(value, message) => {
                (value, message)
            }
            HttpResponseStatus::StatusNoContent(value, message) => (value, message),
            HttpResponseStatus::StatusResetContent(value, message) => (value, message),
            HttpResponseStatus::StatusPartialContent(value, message) => (value, message),
            HttpResponseStatus::StatusMultipleChoices(value, message) => (value, message),
            HttpResponseStatus::StatusMovedPermanently(value, message) => (value, message),
            HttpResponseStatus::StatusFound(value, message) => (value, message),
            HttpResponseStatus::StatusSeeOther(value, message) => (value, message),
            HttpResponseStatus::StatusNoModified(value, message) => (value, message),
            HttpResponseStatus::StatusUseProxy(value, message) => (value, message),
            HttpResponseStatus::StatusUnused(value, message) => (value, message),
            HttpResponseStatus::StatusTemporaryRedirect(value, message) => (value, message),
            HttpResponseStatus::StatusBadRequest(value, message) => (value, message),
            HttpResponseStatus::StatusUnauthorized(value, message) => (value, message),
            HttpResponseStatus::StatusPaymentRequired(value, message) => (value, message),
            HttpResponseStatus::StatusForbidden(value, message) => (value, message),
            HttpResponseStatus::StatusNotFound(value, message) => (value, message),
            HttpResponseStatus::StatusMethodNotAllowed(value, message) => (value, message),
            HttpResponseStatus::StatusNotAcceptable(value, message) => (value, message),
            HttpResponseStatus::StatusProxyAuthenticationRequired(value, message) => {
                (value, message)
            }
            HttpResponseStatus::StatusRequestTimeout(value, message) => (value, message),
            HttpResponseStatus::StatusConflict(value, message) => (value, message),
            HttpResponseStatus::StatusGone(value, message) => (value, message),
            HttpResponseStatus::StatusLengthRequired(value, message) => (value, message),
            HttpResponseStatus::StatusPreconditionFailed(value, message) => (value, message),
            HttpResponseStatus::StatusRequestEntityTooLarge(value, message) => (value, message),
            HttpResponseStatus::StatusRequestUriTooLong(value, message) => (value, message),
            HttpResponseStatus::StatusUnsupportedMediaType(value, message) => (value, message),
            HttpResponseStatus::StatusRequestedRangeNotSatisfiable(value, message) => {
                (value, message)
            }
            HttpResponseStatus::StatusExpectationFailed(value, message) => (value, message),
            HttpResponseStatus::StatusInternalServerError(value, message) => (value, message),
            HttpResponseStatus::StatusNotImplemented(value, message) => (value, message),
            HttpResponseStatus::StatusBadGateway(value, message) => (value, message),
            HttpResponseStatus::StatusServiceUnavailable(value, message) => (value, message),
            HttpResponseStatus::StatusGatewayTimeout(value, message) => (value, message),
            HttpResponseStatus::StatusHttpVersionNotSupported(value, message) => (value, message),
        };
        header::parse(
            self.headers,
            String::from("Content-Length"),
            self.content.len().to_string(),
        );
        let mut response_http = String::from(format!("HTTP/1.1 {} {}", code, message).to_string());
        response_http.push_str(
            format!(
                "\r\nContent-Type: {}",
                header::get_item(&self.headers, HeaderKeys::ContentType)
            )
            .as_str(),
        );
        response_http.push_str(
            format!(
                "\r\nServer: {}",
                header::get_item(&self.headers, HeaderKeys::Server).as_str()
            )
            .as_str(),
        );
        response_http.push_str(
            format!(
                "\r\nDate: {}",
                header::get_item(&self.headers, HeaderKeys::Date)
            )
            .as_str(),
        );
        response_http.push_str(
            format!(
                "\r\nContent-Length: {}",
                header::get_item(&self.headers, HeaderKeys::ContentLength)
            )
            .as_str(),
        );
        response_http.push_str(format!("\r\nConnection: {}", "close").as_str());
        response_http.push_str(format!("\r\n\r\n{}", self.content).as_str());
        response_http
    }
}
