#include <cmath>
#include <iostream>
// color.hpp

class vec3 {
	public:
	vec3(double x, double y, double z) : e{x, y, z} {}

	double x() const { return e[0]; }
	double y() const { return e[1]; }
	double z() const { return e[2]; }

	vec3& operator*=(double t) {
		e[0] *= t;
		e[1] *= t;
		e[2] *= t;

		return *this;
	}

	vec3& operator/=(double t) {
		return *this *= 1/t;
	}
	
	double length() const {
		return sqrt(length_squared());
	}

	double length_squared() const {
		return e[0]*e[0] + e[1]*e[1] + e[2]*e[2];
	}

	private:
		double e[3];
};

inline vec3 operator*(double t, const vec3& v) {
	return vec3(t*v.x(), t*v.y(), t*v.z());
}

inline vec3 operator*(const vec3& v, double t) {
	return t * v;
}

inline vec3 operator/(const vec3& v, double t) {
	return (1/t) * v;
}

inline vec3 operator-(const vec3& u, const vec3& v) {
	return vec3(u.x() - v.x(), u.y() - v.y(), u.z() - v.z());
}

inline vec3 operator+(const vec3& u, const vec3& v) {
	return vec3(u.x() + v.x(), u.y() + v.y(), u.z() + v.z());
}

inline vec3 unit_vector(const vec3& v) {
    return v / v.length();
}

using point3 = vec3;
using color = vec3;

class ray {
	public:
		ray(const point3& origin, const vec3& direction): orig(origin), dir(direction) {}

		const vec3& direction() const {
			return dir;
		}
	private:
		point3 orig;
		vec3 dir;
};

void write_color(std::ostream &out, const color &pixel_color) noexcept {
  auto r = pixel_color.x();
  auto g = pixel_color.y();
  auto b = pixel_color.z();
  // From [0, 1] range to [0, 255];
  int rbyte = static_cast<int>(255.999 * r);
  int gbyte = static_cast<int>(255.999 * g);
  int bbyte = static_cast<int>(255.999 * b);

  out << rbyte << ' ' << gbyte << ' ' << bbyte << '\n';
}

color ray_color(const ray& r) {
    vec3 unit_direction = unit_vector(r.direction());
    auto a = 0.5*(unit_direction.y() + 1.0);
    return (1.0-a)*color(1.0, 1.0, 1.0) + a*color(0.5, 0.7, 1.0);
}

int main(int argc, char *argv[]) {

  auto aspect_ratio = 16.0 / 9.0;
  int image_width = 400;

  int image_height = static_cast<int>(image_width / aspect_ratio);
  image_height = (image_height < 1) ? 1 : image_height;

  // Camera setup

  auto focal_length = 1.0;
  auto viewport_height = 2.0;
  auto viewport_width =
      viewport_height * (static_cast<double>(image_width) / image_height);
  auto camera_center = point3{0, 0, 0};

  auto viewport_u = vec3{viewport_width, 0, 0};
  auto viewport_v = vec3{0, -viewport_height, 0};

  // Delta vectors
  auto pixel_delta_u = viewport_u / image_width;
  auto pixel_delta_v = viewport_v / image_height;

  // location of the upper left pixel;

  auto viewport_upperleft = camera_center - vec3{0, 0, focal_length} -
                            viewport_u / 2 - viewport_v / 2;

  auto pixel00_loc = viewport_upperleft + 0.5 * (pixel_delta_u + pixel_delta_v);

  std::cout << "P3\n" << image_width << ' ' << image_height << "\n255\n";


  for (int j = 0; j < image_height; ++j) {
    std::clog << "\rScanlines remaining: " << (image_height - j) << ' '
              << std::flush;
    for (int i = 0; i < image_width; ++i) {
      /* auto pixel_color = color(static_cast<double>(i) / image_width - 1,
                               static_cast<double>(j) / image_height - 1, 0); */

      auto pixel_center =
          pixel00_loc + (i * pixel_delta_u) + (j * pixel_delta_v);
      auto ray_direction = pixel_center - camera_center;
      ray r{camera_center, ray_direction};

      color pixel_color = ray_color(r);

      write_color(std::cout, pixel_color);
    }
  }

  return 0;
}
