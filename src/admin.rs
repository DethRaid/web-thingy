/// Various administrator functionality
///
/// First use case will be generating API keys. The auth module will provide a way for an authenticated user to generate
/// a new API key. The key will be stored in some database/text file, such that requests to upload, change, or delete an
/// image can check the provided API key against that file
