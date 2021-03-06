// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

pub trait FromProto<T>: Sized {
    fn from_proto(p_val: T) -> Self;
}

pub trait TryFromProto<T>: Sized {
    type Error;

    fn try_from_proto(p_val: T) -> Result<Self, Self::Error>;
}

pub trait TryIntoProto<T>: Sized {
    type Error;

    fn try_into_proto(self) -> Result<T, Self::Error>;
}
