import Vapor

func routes(_ app: Application) throws {
    app.get { req async in
        "Hello World"
    }
}
