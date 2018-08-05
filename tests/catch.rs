
/*
 * TODO: Parsee fail:
 * - both @catch and @get on same method
 */


/*
impl_web! {
    impl Buggy {
        /// @get("/")
        fn index(&self) -> impl Future<Item = String, Error = io::Error> + Send {
            Ok("hello".to_string()).into_future()
        }

        /// @catch
        /// @catch(4xx)
        /// @catch(400, 404)
        /// @catch(500)
        /// @catch(500, 500)

        /// @catch(404)
        fn not_found(&self) -> impl Future<Item = String, Error = io::Error> + Send {
            Ok("hello".to_string()).into_future()
        }
    }

    impl Catch {
    }
}
 */
