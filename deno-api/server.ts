const server = Deno.listen({port: 8080})
console.log(`Deno web server running at: http://localhost:8080`)

for await (const conn of server) {
    serveHttp(conn);
}

async function serveHttp(conn: Deno.Conn) {
    const httpConn = Deno.serveHttp(conn);

    for await (const requestEvent of httpConn) {
        const body = `Hello World, from Deno!`

        requestEvent.respondWith(
            new Response(body, {
                status: 200
            })
        )

        console.log('Response sent successfully!')
    }
}