initSidebarItems({"enum":[["AccessControlAllowOrigin","The `Access-Control-Allow-Origin` response header, part of CORS"],["ByteRangeSpec","Each `Range::Bytes` header can contain one or more `ByteRangeSpecs`. Each `ByteRangeSpec` defines a range of bytes to fetch"],["CacheDirective","`CacheControl` contains a list of these directives."],["Charset","A Mime charset."],["ConnectionOption","Values that can be in the `Connection` header."],["ContentRangeSpec","Content-Range, described in RFC7233"],["DispositionParam","A parameter to the disposition type."],["DispositionType","The implied disposition of the content of the HTTP body."],["Encoding","A value to represent an encoding used in `Transfer-Encoding` or `Accept-Encoding` header."],["Expect","The `Expect` header."],["IfMatch","`If-Match` header, defined in RFC7232"],["IfNoneMatch","`If-None-Match` header, defined in RFC7232"],["IfRange","`If-Range` header, defined in RFC7233"],["MediaDesc","A Media Descriptors Enum based on: [https://www.w3.org/TR/html401/types.html#h-6.13][url]"],["Pragma","The `Pragma` header defined by HTTP/1.0."],["Preference","Prefer contains a list of these preferences."],["ProtocolName","A protocol name used to identify a specific protocol. Names are case-sensitive except for the `WebSocket` value."],["Range","`Range` header, defined in RFC7233"],["RangeUnit","Range Units, described in RFC7233"],["ReferrerPolicy","`Referrer-Policy` header, part of Referrer Policy"],["RelationType","A Link Relation Type Enum based on: RFC5988"],["RetryAfter","The `Retry-After` header."],["Vary","`Vary` header, defined in RFC7231"]],"fn":[["q","Convenience function to create a `Quality` from a float or integer."],["qitem","Convenience function to wrap a value in a `QualityItem` Sets `q` to the default 1.0"]],"mod":[["parsing","Utility functions for Header implementations."]],"struct":[["Accept","`Accept` header, defined in RFC7231"],["AcceptCharset","`Accept-Charset` header, defined in RFC7231"],["AcceptEncoding","`Accept-Encoding` header, defined in RFC7231"],["AcceptLanguage","`Accept-Language` header, defined in RFC7231"],["AcceptRanges","`Accept-Ranges` header, defined in RFC7233"],["AccessControlAllowCredentials","`Access-Control-Allow-Credentials` header, part of CORS"],["AccessControlAllowHeaders","`Access-Control-Allow-Headers` header, part of CORS"],["AccessControlAllowMethods","`Access-Control-Allow-Methods` header, part of CORS"],["AccessControlExposeHeaders","`Access-Control-Expose-Headers` header, part of CORS"],["AccessControlMaxAge","`Access-Control-Max-Age` header, part of CORS"],["AccessControlRequestHeaders","`Access-Control-Request-Headers` header, part of CORS"],["AccessControlRequestMethod","`Access-Control-Request-Method` header, part of CORS"],["Allow","`Allow` header, defined in RFC7231"],["Authorization","`Authorization` header, defined in RFC7235"],["Basic","Credential holder for Basic Authentication"],["Bearer","Token holder for Bearer Authentication, most often seen with oauth"],["CacheControl","`Cache-Control` header, defined in RFC7234"],["Connection","`Connection` header, defined in RFC7230"],["ContentDisposition","A `Content-Disposition` header, (re)defined in RFC6266."],["ContentEncoding","`Content-Encoding` header, defined in RFC7231"],["ContentLanguage","`Content-Language` header, defined in RFC7231"],["ContentLength","`Content-Length` header, defined in RFC7230"],["ContentLocation","`Content-Location` header, defined in RFC7231"],["ContentRange","`Content-Range` header, defined in RFC7233"],["ContentType","`Content-Type` header, defined in RFC7231"],["Cookie","`Cookie` header, defined in RFC6265"],["CookieIter","Iterator for cookie."],["Date","`Date` header, defined in RFC7231"],["ETag","`ETag` header, defined in RFC7232"],["EntityTag","An entity tag, defined in RFC7232"],["Expires","`Expires` header, defined in RFC7234"],["Formatter","A formatter used to serialize headers to an output stream."],["From","`From` header, defined in RFC7231"],["HeaderView","Returned with the `HeadersItems` iterator."],["Headers","A map of header fields on requests and responses."],["HeadersItems","An `Iterator` over the fields in a `Headers` map."],["Host","The `Host` header."],["HttpDate","A timestamp with HTTP formatting and parsing"],["IfModifiedSince","`If-Modified-Since` header, defined in RFC7232"],["IfUnmodifiedSince","`If-Unmodified-Since` header, defined in RFC7232"],["LanguageTag","A language tag as described in BCP47."],["LastEventId","`Last-Event-ID` header, defined in RFC3864"],["LastModified","`Last-Modified` header, defined in RFC7232"],["Link","The `Link` header, defined in RFC5988"],["LinkValue","A single `link-value` of a `Link` header, based on: RFC5988"],["Location","`Location` header, defined in RFC7231"],["Origin","The `Origin` header."],["Prefer","`Prefer` header, defined in RFC7240"],["PreferenceApplied","`Preference-Applied` header, defined in RFC7240"],["Protocol","Protocols that appear in the `Upgrade` header field"],["ProxyAuthorization","`Proxy-Authorization` header, defined in RFC7235"],["Quality","Represents a quality used in quality values."],["QualityItem","Represents an item with a quality value as defined in RFC7231."],["Raw","A raw header value."],["Referer","`Referer` header, defined in RFC7231"],["Server","`Server` header, defined in RFC7231"],["SetCookie","`Set-Cookie` header, defined RFC6265"],["StrictTransportSecurity","`StrictTransportSecurity` header, defined in RFC6797"],["Te","`TE` header, defined in RFC7230"],["TransferEncoding","`Transfer-Encoding` header, defined in RFC7230"],["Upgrade","`Upgrade` header, defined in RFC7230"],["UserAgent","`User-Agent` header, defined in RFC7231"],["Warning","`Warning` header, defined in RFC7234"]],"trait":[["Header","A trait for any object that will represent a header field and value."],["Scheme","An Authorization scheme to be used in the header."]]});